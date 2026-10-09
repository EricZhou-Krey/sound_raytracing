pub mod acoustic_logic;
pub mod app;
pub mod asset;
pub mod command;
pub mod component;
pub mod demo;
pub mod renderer;
pub mod resource;
pub mod system;

/*
ORDER OF OPERATIONS (many, MANY mini steps inbetween):

1.5 -> Debug lines and points implementation, hook terminal commands to loading certain scenes, by scpecifitying a Scene Entity and using a terminal filepath to get the resource and load into it via a event trigger, for debug line and debug point similiarly, hook onto terminal and define a seperate system that triggers on an event to spawn debug drawables

2. Render audio rays, for debug purposes, display bouncing, speed, collsisions, reflection rays, transmitted rays and etc.
3. Hook onto audio and map ray collsisons and %ray coverage to the audio output.

*/
