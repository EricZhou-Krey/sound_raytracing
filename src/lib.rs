pub mod acoustic_logic;
pub mod app;
pub mod asset;
pub mod command;
pub mod component;
pub mod renderer;
pub mod resource;
pub mod system;

/*
ORDER OF OPERATIONS (many, MANY mini steps inbetween):

1. Get renderer working, configure, meshes, materials, bind groups, etc to display a lighted environment model from a blender file.
    -> Need to refactor so an MeshId, TextureId, MaterialId is stored for each CPU and corresponding GPU resource
2. Render audio rays, for debug purposes, display bouncing, speed, collsisions, reflection rays, transmitted rays and etc.
3. Hook onto audio and map ray collsisons and %ray coverage to the audio output.

*/
