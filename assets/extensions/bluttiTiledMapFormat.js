var bluttiMapFormat = {
    name: "Blutti map format",
    extension: "json",


    color: function(map, prop) {
        return this.COLORS[map.property(prop).value];
    },

    // TODO: Read list of monsters from a layer
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
                    var obj = layer.objectAt(x);
                    var tile = obj["tile"];
                    var monster = {
                        position: {
                            x: obj["x"],
                            y: obj["y"]
                        },
                        sprite: tile["id"],
                        movement: obj.property("movement")
                    };
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

