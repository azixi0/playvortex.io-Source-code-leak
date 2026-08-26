// -- leaked by @azixi0 on github
#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

@group(#{MATERIAL_BIND_GROUP}) @binding(100) var stud_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(101) var stud_sampler: sampler;
@group(#{MATERIAL_BIND_GROUP}) @binding(102) var inlet_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(103) var inlet_sampler: sampler;

struct StudUniform {
    apply_stud: f32,
    apply_inlet: f32,
};
@group(#{MATERIAL_BIND_GROUP}) @binding(104) var<uniform> stud_uniform: StudUniform;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

    let base_rgb = pbr_input.material.base_color.rgb;
    let tint = max(base_rgb, vec3<f32>(0.05, 0.05, 0.05));

#ifdef VERTEX_UVS_B
    let apply_stud = in.uv_b.x;
    let apply_inlet = in.uv_b.y;
#else
    let apply_stud = stud_uniform.apply_stud;
    let apply_inlet = stud_uniform.apply_inlet;
#endif

    if apply_stud > 0.5 {
        let tex = textureSample(stud_texture,  stud_sampler,  in.uv);
        pbr_input.material.base_color = vec4<f32>(
            mix(base_rgb, tint * tex.rgb, tex.a),
            pbr_input.material.base_color.a
        );
    } else if apply_inlet > 0.5 {
        let tex = textureSample(inlet_texture, inlet_sampler, in.uv);
        pbr_input.material.base_color = vec4<f32>(
            mix(base_rgb, tint * tex.rgb, tex.a),
            pbr_input.material.base_color.a
        );
    }

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    return out;
}
