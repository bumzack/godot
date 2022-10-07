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

    <title>Hello, world!</title>

    <script>
        $(document).ready(function () {
            $("#renderscene").click(function () {
                $("#renderscene").prop("disabled", true);
                const uri = 'ws://localhost:3030/openwebsocket';
                const ws = new WebSocket(uri);

                ws.onopen = function () {
                    $("#statustext").html( "Running!");
                    let width = parseInt($("#widthInput").val());
                    let height = parseInt($("#heightInput").val());

                    let world = {
                        "width": width,
                        "height": height
                    };
                    console.log("sending worldscene ", JSON.stringify(world));
                    let w = JSON.stringify(world);
                    ws.send(w);
                };

                ws.onmessage = function (msg) {
                    prcoess_message(msg.data);
                };

                ws.onclose = function () {
                    console.log("websocket closed");
                    $("#statustext").html( "finished!");
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

        });


    </script>
</head>
<body>
<div class="container">
    <div class="row">
        <div class="col">
            <h1>Hello, world!</h1>
        </div>
    </div>

    <div class="row">
        <div class="col-2">
            <form>
                <div class="mb-3">
                    <label for="widthInput" class="form-label">Width</label>
                    <input type="number" class="form-control" id="widthInput" aria-describedby="emailHelp" value="200">
                </div>
                <div class="mb-3">
                    <label for="heightInput" class="form-label">Height</label>
                    <input type="number" class="form-control" id="heightInput" aria-describedby="emailHelp" value="160">
                </div>
                <!--                <div class="mb-3 form-check">-->
                <!--                    <input type="checkbox" class="form-check-input" id="exampleCheck1">-->
                <!--                    <label class="form-check-label" for="exampleCheck1">Check me out</label>-->
                <!--                </div>-->
                <button id="renderscene" type="button" class="btn btn-primary">Render scene</button>
            </form>

            <br/>
            <div ><p id="statustext"></p>
            </div>
        </div>
        <div class="col">
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
