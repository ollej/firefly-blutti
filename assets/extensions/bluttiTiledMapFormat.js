function filteredClone(obj, ...keys) {
    let clone = {};
    for (let key of keys) {
        clone[key] = obj.resolvedProperty(key)
    }
    return clone;
}

var bluttiMapFormat = {
    name: "Blutti map format",
    extension: "json",

    color: function(map, prop) {
        return this.COLORS[map.property(prop).value];
    },

    write: function(map, fileName) {
        COLORS = [
          "Black",
          "Purple",
          "Red",
          "Orange",
          "Yellow",
          "LightGreen",
          "Green",
          "DarkGreen",
          "DarkBlue",
          "Blue",
          "LightBlue",
          "Cyan",
          "White",
          "LightGray",
          "Gray",
          "DarkGray"
        ];
        COLLISION = [
          "Blocking",
          "Deadly",
          "None",
        ];
        MOVEMENT = [
          "TurnsAtEdge",
          "FollowsPlayer",
          "Moving",
          "Flying",
        ];
        var m = {
            background_color: this.COLORS[map.property("background_color")["value"]],
            font_color: this.COLORS[map.property("font_color")["value"]],
            particle_chance: map.property("particle_chance"),
            particle_sprite: map.property("particle_sprite"),
            stars: map.property("stars"),
            start_position: map.property("start_position")["value"],
            monsters: [],
            tiles: []
        };

        for (var i = 0; i < map.layerCount; ++i) {
            var layer = map.layerAt(i);
            if (layer.isTileLayer) {
                for (y = 0; y < layer.height; ++y) {
                    for (x = 0; x < layer.width; ++x) {
                        m.tiles.push(layer.cellAt(x, y).tileId + 1);
                    }
                }
            }
            if (layer.isObjectLayer) {
                for (x = 0; x < layer.objects.length; ++x) {
                    const obj = layer.objectAt(x);
                    let monster = filteredClone(obj, "gravity", "frames", "velocity");
                    let reverse_sprite = obj.resolvedProperty("reverse_sprite");
                    if (reverse_sprite == undefined || reverse_sprite == -1) {
                        reverse_sprite = obj["tile"]["id"] + monster["frames"];
                    }

                    Object.assign(monster, {
                        "collision": this.COLLISION[obj.resolvedProperty("collision")["value"]],
                        "movement": this.MOVEMENT[obj.resolvedProperty("movement")["value"]],
                        "position": {
                            x: obj["x"],
                            y: obj["y"]
                        },
                        "reverse_sprite": reverse_sprite,
                        "sprite": obj["tile"]["id"],
                        "velocity": {
                            x: obj.resolvedProperty("velocity")["value"]["x"] || 0.0,
                            y: obj.resolvedProperty("velocity")["value"]["y"] || 0.0
                        }
                    });
                    m.monsters.push(monster);
                }
            }
        }

        var file = new TextFile(fileName, TextFile.WriteOnly);
        file.write(JSON.stringify(m, null, 4));
        file.commit();
    },
}

tiled.registerMapFormat("blutti", bluttiMapFormat)

