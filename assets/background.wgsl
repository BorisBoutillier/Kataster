// This shader is inspired by Star Nest by Pablo Roman Andrioli:
// https://www.shadertoy.com/view/XlfGRj

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct BackgroundMaterial {
    time: f32,
};

@group(1) @binding(0)
var<uniform> background: BackgroundMaterial;

const iterations = 17;
const formuparam = 0.53;

const volsteps = 20;
const stepsize = 0.1;

const zoom = 0.800;
const tile = 0.850;
const speed = 0.003;

const brightness = 0.0015;
const darkmatter = 0.600;
const distfading = 0.730;
const saturation = 0.850;

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    let dir = vec3<f32>(in.uv * zoom, 1.0);
    let time = background.time * speed + 0.25;
    var from_ = vec3<f32>(1.0, 0.5, 0.5);
    from_ = from_ + vec3<f32>(time * 2., time, -2.);
    
    // volumetric rendering
    var s = 0.1;
    var fade = 1.0;
    var v = vec3<f32>(0.);
    for (var r = 0; r < volsteps; r = r + 1) {
        var p = from_ + s * dir * 0.5;
        p = abs(vec3<f32>(tile) - (p % vec3<f32>(tile * 2.0)));

        var pa = 0.0;
        var a = 0.0;
        for (var i = 0; i < iterations; i = i + 1) {
            p = abs(p) / dot(p, p) - formuparam; // the magic formula
            a = a + abs(length(p) - pa); // absolute sum of average change
            pa = length(p);
        }

        let dm = max(0.0, darkmatter - a * a * 0.001); // dark matter
        a = a * a * a; // add contrast
        if r > 6 {
            fade = fade * (1. - dm); // dark matter, don't render near
        }
        v = v + fade;
        v = v + vec3<f32>(s, s * s, s * s * s * s) * a * brightness * fade; // coloring based on distance
        fade = fade * distfading; // distance fading;
        s = s + stepsize;
    }
    v = mix(vec3<f32>(length(v)), v, saturation); // color_adjust
    return vec4<f32>(v * 0.0006, 1.0);
}
