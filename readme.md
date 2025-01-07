# Test Plugin

*This is the test repo for the embedded db/inference plugin.* 
The plugin uses an embedded ONNX runtime to load and execute ONNX models from within an unreal project, with support for in-memory db queries and speech to text input parsing.

## Install

```
```

## Usage

```
```

## Components
1. Inference Handler. This fn fetches the scene context stored in-memory, combining scene data and instruction into a single string, which is then passed to the model as a raw ndarray<float>. Output string is then serialized into JSON and returned.

## TODO
### Inference
 - [x] Implement basic inference handler.
 - [] Implement model loading and inference from within ue5 project.
 - [] Implement whisper model for STT functionality.
### Database
 - [] Implement db handler.
 - [] Implement Vision Cone Filter logic to identify visible objects and geometry within the NPC's field of view.
 - [] Collect and serialize scene and object data for database storage.
 - [] Connect Vision Cone Filter to SurrealDB for runtime updates.
 ### Engine Integration
 - [] Create UE5 Blueprints for common NPC actions (e.g., animations, object interaction).
 
## Contributing

PRs not accepted outside of organization.

## License

GNU Affero General Public License 3.0 @ Kelly Gibson