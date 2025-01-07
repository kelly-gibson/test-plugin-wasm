# Test Plugin

This is the test repo for the embedded db/inference plugin. Will be compiled into wasm.

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
 - [x] Implement basic inference handler
 - [] Implemet model loading and inference from within ue5 project.
### Database
 - [] Implement db handler.
 - [] Implement Vision Cone Filter logic to identify visible objects and geometry within the NPC's field of view.
 - [] Collect and serialize scene and object data for database storage.
 - [] Connect Vision Cone Filter to SurrealDB for runtime updates.
 
## Contributing

PRs not accepted.

## License

GNU Affero General Public License 3.0 @ Kelly Gibson