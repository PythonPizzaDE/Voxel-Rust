#version 330 core

in vec3 Color;
in vec3 Normal;
in vec3 Position;

out vec4 FragColor;

void main()
{
    vec3 light_position = vec3(30.0f, 50.0f, 30.0f);
    float ambient_light = 0.75f;

    float angle = dot(Normal, normalize(light_position - Position));
    float dynamic_light = (1.0f - ambient_light) * angle;
    dynamic_light = max(0, dynamic_light);
    FragColor = vec4(Color * (ambient_light + dynamic_light), 1.0f);
}
