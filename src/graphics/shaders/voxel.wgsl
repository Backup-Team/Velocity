struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] colour:   vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]]       colour:   vec4<f32>;
};

[[stage(vertex)]]
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.position = vec4<f32>(input.position, 1.0);
    output.colour   = vec4<f32>(input.colour  , 1.0);

    return output;
}

[[stage(fragment)]]
fn main(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    return input.colour;
}
