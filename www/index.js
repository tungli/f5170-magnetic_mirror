import { Config, Trajectory, psi_1 } from "@tungli/wasm-f5170_magnetic_mirror_ex1";

var config = Config.empty();

config.b0 =               parseFloat(document.getElementById("B0").value);
config.l =                parseFloat(document.getElementById("L").value);
config.vel_perp =         parseFloat(document.getElementById("vel_perp").value);
config.vel_par =          parseFloat(document.getElementById("vel_par").value);
config.n_steps =          parseInt(document.getElementById("n_steps").value);

document.getElementById("B0").addEventListener("change", function () {
    config.b0 = parseFloat(this.value);
});
document.getElementById("L").addEventListener("change", function () {
    config.l = parseFloat(this.value);
});
document.getElementById("vel_perp").addEventListener("change", function () {
    config.vel_perp = parseFloat(this.value);
});
document.getElementById("vel_par").addEventListener("change", function () {
    config.vel_par = parseFloat(this.value);
});
document.getElementById("n_steps").addEventListener("change", function () {
    config.n_steps = parseFloat(this.value);
});


var part_anim_steps =  parseInt(document.getElementById("anim_steps").value);
document.getElementById("anim_steps").addEventListener("change", function () {
    part_anim_steps = parseInt(this.value);
});


var but0 = document.getElementById("gen_traj");
but0.addEventListener("click", gen_traj);

var traj_length = 0;
var do_anim = false;
var p_at_index = 0;
var x = [], y = [], z = [];
var px = [], py = [], pz = [];
var traj_plot = {};

var traj;
function gen_traj () {
    console.log(config);
    traj = Trajectory.new(Config.from_struct(config));
    traj_length = config.n_steps;
    
    p_at_index = 0;
    x = [], y = [], z =[];
    for(var i=0; i<=config.n_steps; i++) {
        x.push(traj.at(i).x());
        y.push(traj.at(i).y());
        z.push(traj.at(i).z());
    }
    
    traj_plot = {
        type: 'scatter3d',
        x: x,
        y: y,
        z: z,
        mode: 'lines',
        opacity: 0.7,
        line: {
           width: 2,
           color: 4,
           colorscale: 'Turbo'
        }
    };
    
    px = [x[0]], py = [y[0]], pz = [z[0]];
    var particle = {
        type: 'scatter3d',
        x: px,
        y: py,
        z: pz,
    };
    
    Plotly.newPlot('plot3d', [traj_plot, particle], {showlegend: false});
    do_anim = false;
}

function update () {
    px[0] = x[p_at_index];
    py[0] = y[p_at_index];
    pz[0] = z[p_at_index];
    
    Plotly.animate('plot3d', {
        data: [traj_plot, {x: px, y: py, z: pz}]
    }, {
        transition: {
            duration: 0,
        },
        frame: {
            duration: 0,
            redraw: true,
        },
        showlegend: false
    });

    p_at_index += part_anim_steps;

    if (do_anim && (p_at_index < traj_length)) {
        let i = p_at_index;
        document.getElementById("p_time").value = traj.at(i).t;
        document.getElementById("mag_moment").value = traj.at(i).mag_moment();
        document.getElementById("energy").value = traj.at(i).energy();
        document.getElementById("vperp").value = traj.at(i).v_perp();
        document.getElementById("field_mag").value = traj.at(i).field_mag;
        document.getElementById("psi").value = psi_1(traj.at(i), Config.from_struct(config));
        requestAnimationFrame(update);
    }
    else if (p_at_index >= config.n_steps) {
        do_anim = false;
        p_at_index = 0;
        px[0] = x[0], py[0] = y[0], pz[0] = z[0]; 
    }
}

var but1 = document.getElementById("run_anim");
but1.addEventListener("click", runAnim);

var but2 = document.getElementById("stop_anim");
but2.addEventListener("click", stopAnim);

function runAnim () {
    if (!do_anim) {
        do_anim = true;
        requestAnimationFrame(update);
    }
}

function stopAnim () {
    do_anim = false;
}

