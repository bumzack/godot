pub static INDEX_HTML: &str = r###"<!doctype html>
<html lang="en">
<head>
    <!-- Required meta tags -->
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">

    <!-- Bootstrap CSS -->
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css" rel="stylesheet"
          integrity="sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC" crossorigin="anonymous">

    <script src="https://ajax.googleapis.com/ajax/libs/jquery/3.6.0/jquery.min.js"></script>

    <title>Hello, raytracer!</title>

    <script>
        $(document).ready(function () {

            const scenes = [];

            loadScenes();


            $("#renderscene").click(function () {

                const scene_id = parseInt($('#sceneSelect').find(":selected").val());
                console.log("scene id ", scene_id);

                let scene_data = {
                    "id": parseInt(scene_id),
                    "up": {
                        "x": parseFloat($("#upX").val()),
                        "y": parseFloat($("#upY").val()),
                        "z": parseFloat($("#upZ").val()),
                        "w": 0
                    },
                    "from": {
                        "x": parseFloat($("#fromX").val()),
                        "y": parseFloat($("#fromY").val()),
                        "z": parseFloat($("#fromZ").val()),
                        "w": 0
                    },
                    "to": {
                        "x": parseFloat($("#toX").val()),
                        "y": parseFloat($("#toY").val()),
                        "z": parseFloat($("#toZ").val()),
                        "w": 0
                    },
                    "width": parseInt($("#widthInput").val()),
                    "height": parseInt($("#heightInput").val()),
                    "fov": parseFloat($("#fovInput").val()),
                    "antialiasing": parseInt($("#antialiasValue").val()),
                    "shadows": true,
                    "name": "ignore",
                    "size_area_light":  parseInt($("#arealightsize").val())
                };

                const ctx = $("#canvas")[0].getContext('2d');
                ctx.canvas.width = parseInt($("#widthInput").val());
                ctx.canvas.height = parseInt($("#heightInput").val());

                $("#renderscene").prop("disabled", true);
                const uri = 'ws://localhost:3030/openwebsocket';
                const ws = new WebSocket(uri);

                ws.onopen = function () {
                    $("#statustext").html("Running!");

                    console.log("sending scene_data ", JSON.stringify(scene_data));
                    let scene_data_string = JSON.stringify(scene_data);
                    ws.send(scene_data_string);
                };

                ws.onmessage = function (msg) {
                    prcoess_message(msg.data);
                };

                ws.onclose = function () {
                    console.log("websocket closed");
                    $("#statustext").html("finished!");
                    $("#renderscene").prop("disabled", false);
                };
            });

            function prcoess_message(msg) {
                const ctx = $("#canvas")[0].getContext('2d');
                const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height)

                var tile = JSON.parse(msg);
                var it = 0;
                console.log(" tile idx ", tile.idx);
                for (let i = 0; i < tile.points.length; i++) {
                    const point = tile.points[i];
                    var idx = parseInt(point.y * canvas.width * 4 + point.x * 4);

                    var r = parseInt(point.c.r * 255);
                    var g = parseInt(point.c.g * 255);
                    var b = parseInt(point.c.b * 255);

                    imageData.data[idx] = r;
                    imageData.data[idx + 1] = g;
                    imageData.data[idx + 2] = b;
                    imageData.data[idx + 3] = 255;
                }
                console.log("putting data back");
                ctx.putImageData(imageData, 0, 0, 0, 0, canvas.width, canvas.height);
            }

            function loadScenes() {
                console.log("loading scenes");

                $.ajax({
                    url: "/scenes",
                    type: 'get',
                    dataType: 'json',
                    success: function (data) {
                        const all_scenes = JSON.parse(data);
                        console.log("all_scenes  " + JSON.stringify(all_scenes, null, 4));

                        $("#sceneSelect option").each(function () {
                            $(this).remove();
                        });
                        // scenes = [];

                        $.each(all_scenes.scenes, function (i, scene) {
                            $('#sceneSelect').append($('<option>', {
                                value: scene.id,
                                text: scene.name
                            }));
                            scenes.push(scene);
                        });

                    }
                });
            }

            $('#sceneSelect').on('change', function () {
                const scene_id = $('#sceneSelect').find(":selected").val();
                console.log("scene id ", scene_id);
                console.log("scenes ", JSON.stringify(scenes, null, 4));

                const scene = scenes.find((s) => {
                    console.log("comparing " + s.id + " with  scene_id " + scene_id);
                    return parseInt(s.id) === parseInt(scene_id);
                });
                console.log("scene  ", JSON.stringify(scene, null, 4));


                $("#widthInput").val(scene.scene_data.width);
                $("#heightInput").val(scene.scene_data.height);

                $("#fovInput").val(scene.scene_data.fov);


                $("#upX").val(scene.scene_data.up.x);
                $("#upY").val(scene.scene_data.up.y);
                $("#upZ").val(scene.scene_data.up.z);

                $("#fromX").val(scene.scene_data.from.x);
                $("#fromY").val(scene.scene_data.from.y);
                $("#fromZ").val(scene.scene_data.from.z);

                $("#toX").val(scene.scene_data.to.x);
                $("#toY").val(scene.scene_data.to.y);
                $("#toZ").val(scene.scene_data.to.z);


            });
        });
    </script>
</head>
<body>
<div class="container-fluid">
    <div class="row">
        <div class="col-4">
            <h1>Hello, raytracer!</h1>
        </div>
    </div>

    <div class="row">
        <div class="col-3">
            <form class="row g-3">
                <div class="mb-3">
                    <label for="sceneSelect" class="form-label">Scene</label>
                    <select id="sceneSelect" class="form-select" size="3">
                    </select>
                </div>

                <div class="col-md-4">
                    <label for="widthInput" class="form-label">width</label>
                    <input type="number" class="form-control" id="widthInput">
                </div>
                <div class="col-md-4">
                    <label for="heightInput" class="form-label">height</label>
                    <input type="number" class="form-control" id="heightInput">
                </div>
                <div class="col-md-4">
                    <label for="fovInput" class="form-label">FoV</label>
                    <input type="number" class="form-control" id="fovInput">
                </div>

                <div class="col-4">
                    <label for="fromX" class="form-label">From.X</label>
                    <input type="number" class="form-control" id="fromX">
                </div>
                <div class="col-4">
                    <label for="fromY" class="form-label">From.Y</label>
                    <input type="number" class="form-control" id="fromY">
                </div>
                <div class="col-4">
                    <label for="fromZ" class="form-label">From.Z</label>
                    <input type="number" class="form-control" id="fromZ">
                </div>

                <div class="col-4">
                    <label for="toX" class="form-label">To.X</label>
                    <input type="number" class="form-control" id="toX">
                </div>
                <div class="col-4">
                    <label for="toY" class="form-label">To.Y</label>
                    <input type="number" class="form-control" id="toY">
                </div>
                <div class="col-4">
                    <label for="toZ" class="form-label">To.Z</label>
                    <input type="number" class="form-control" id="toZ">
                </div>

                <div class="col-4">
                    <label for="upX" class="form-label">Up.X</label>
                    <input type="number" class="form-control" id="upX">
                </div>
                <div class="col-4">
                    <label for="upY" class="form-label">Up.Y</label>
                    <input type="number" class="form-control" id="upY">
                </div>
                <div class="col-4">
                    <label for="upZ" class="form-label">Up.Z</label>
                    <input type="number" class="form-control" id="upZ">
                </div>


                <div class="col-6">
                    <div class="form-check">
                        <input class="form-check-input" type="checkbox" id="antialiasBool">
                        <label class="form-check-label" for="antialiasBool">
                            Antialiasing
                        </label>
                    </div>
                </div>
                <div class="col-6">
                    <select id="antialiasValue" class="form-select">
                        <option value="2">2</option>
                        <option value="3">3</option>
                    </select>
                </div>
                <div class="col-6">

                    <div class="form-check">
                        <input class="form-check-input" type="checkbox" id="shadows">
                        <label class="form-check-label" for="shadows">
                            Shadows
                        </label>
                    </div>

                </div>
                <div class="col-6">
                    <label for="arealightsize" class="form-label">Area lights size</label>
                    <input type="number" class="form-control" id="arealightsize">
                </div>
                <button id="renderscene" type="button" class="btn btn-primary">Render scene</button>

            </form>
            <br/>
            <div><p id="statustext"></p>
            </div>
        </div>
        <div class="col-9">
            <canvas id="canvas" width="800" height="600"></canvas>
        </div>
    </div>
</div>
<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/js/bootstrap.bundle.min.js"
        integrity="sha384-MrcW6ZMFYlzcLA8Nl+NtUVF0sA7MsXsP1UyJoMp4YLEuNSfAP+JcXn/tWtIaxVXM"
        crossorigin="anonymous"></script>

</body>
</html>

"###;
