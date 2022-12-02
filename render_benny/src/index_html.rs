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

            $("#renderscene").click(function () {
                const ctx = $("#canvas")[0].getContext('2d');
                ctx.canvas.width = parseInt($("#widthInput").val());
                ctx.canvas.height = parseInt($("#heightInput").val());

                $("#renderscene").prop("disabled", true);
                const uri = 'ws://localhost:3030/openwebsocket';
                const ws = new WebSocket(uri);

                let scene_data = {
                    "up": {
                        "x": 1.0,
                        "y": 1.0,
                        "z": 1.0,
                        "w": 0
                    },
                    "from": {
                        "x": 1.0,
                        "y": 1.0,
                        "z": 1.0,
                        "w": 0
                    },
                    "to": {
                        "x": 1.0,
                        "y": 1.0,
                        "z": 1.0,
                        "w": 0
                    },
                    "width": 800,
                    "height": 600,
                    "fov": 0.7
                };

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
                //console.log(`msg    ${JSON.stringify(msg, null, 4)}`);
                //console.log(`canvas    ${JSON.stringify($("#canvas"), null, 4)}`);

                let canvas = $("#canvas")[0];
                const ctx = canvas.getContext('2d');
                var image = JSON.parse(msg);

                canvas.width = image.width;
                canvas.height = image.height;

                const imageData = new ImageData(image.width, image.height)

                //console.log(`msg: ${JSON.stringify(image, null, 4)}`);
                //console.log(`image.pixels.length: ${image.pixels.length}`);

                for (x = 0; x < image.width; x++) {
                    for (y = 0; y < image.height; y++) {
                        var idx = (y * image.width + x);
                        const point = image.pixels[idx];
                        //console.log(`idx  ${idx}, point  ${JSON.stringify(point, null, 4)}`);

                        idx = idx * 4;
                        imageData.data[idx] = point.r;
                        imageData.data[idx + 1] = point.g;
                        imageData.data[idx + 2] = point.b;
                        imageData.data[idx + 3] = point.a;
                    }
                }
                console.log("putting data back");
                ctx.putImageData(imageData, 0, 0, 0, 0, canvas.width, canvas.height);
                console.log(`width ${canvas.width}, height ${canvas.height}`);
            }
        });
    </script>
</head>
<body>
<div class="container-fluid">
    <div class="row">
        <div class="col-4">
            <h1>Hello, 3D renderer!</h1>
        </div>
    </div>

    <div class="row">
        <div class="col-3">
            <form class="row g-3">
                <div class="mb-3">
                    i dont know
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

