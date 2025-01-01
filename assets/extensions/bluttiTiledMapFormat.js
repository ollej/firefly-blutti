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
            stars: map.property("stars"),
            start_position: map.property("start_position")["value"],
            monsters: [],
            tiles: []
        };

        var monster1 = map.property("monster1")["value"];
        if (monster1) {
            monster1["position"] = monster1["position"]["value"];
            m.monsters.push(monster1);
        }
        var monster2 = map.property("monster2")["value"];
        if (monster2["sprite"]) {
            m.monsters.push(monster2);
        }

        for (var i = 0; i < map.layerCount; ++i) {
            var layer = map.layerAt(i);
            if (layer.isTileLayer) {
                var tiles = [];
                for (y = 0; y < layer.height; ++y) {
                    for (x = 0; x < layer.width; ++x)
                        m.tiles.push(layer.cellAt(x, y).tileId + 1);
                }
            }
        }

        var file = new TextFile(fileName, TextFile.WriteOnly);
        file.write(JSON.stringify(m));
        file.commit();
    },
}

tiled.registerMapFormat("blutti", bluttiMapFormat)

