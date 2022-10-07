pub static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <title>Warp Chat</title>
</head>
<body>
<h1>Warp chat</h1>
<div id="chat">
    <p><em>Connecting...</em></p>
</div>
<div>
    <input type="text" id="text"/>
    <button type="button" id="send">Send</button>
    <br/><br/>
    <button type="start render" id="start_render">Start rendering</button>

    <div>
        <canvas id="canv" width="120" height="80"  style="border: 5px solid blue;" >
    </div>
</div>

<script type="text/javascript">


    const chat = document.getElementById('chat');
    const text = document.getElementById('text');
    const start_render_btn = document.getElementById('start_render');

    start_render_btn.onclick = function () {
        // console.log("clock start render");
        // var req = {"name": "bumzack", "rate": 23};
        //
        // var url = "http://localhost:3030/render"
        // var xhr = new XMLHttpRequest();
        // xhr.open("POST", url, true);
        // xhr.setRequestHeader('Content-Type', 'application/json');
        // xhr.send(JSON.stringify(req));

          const uri = 'ws://localhost:3030/chat';
          const ws = new WebSocket(uri);

            ws.onopen = function () {
                chat.innerHTML = '<p><em>Connected!</em></p>';
            };

            ws.onmessage = function (msg) {
                message(msg.data);
            };

            ws.onclose = function () {
                chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
            };

            send.onclick = function () {
                const msg = text.value;
                ws.send(msg);
                text.value = '';
                message('<You>: ' + msg);
            };

    };





  //  draw();

    function message(msg) {
        var canvas = document.getElementById('canv');
        var ctx = canvas.getContext('2d');
        var imageData = ctx.getImageData(0,0, canvas.width, canvas.height)
        var data = imageData.data;
        console.log(`canvas w x h ${canvas.width}  x  ${canvas.height}  `);

        let arr = JSON.parse(msg);
        let it = 0;
        arr.forEach(function(point) {
            let idx = parseInt(point.y * canvas.width * 4 + point.x*4);

            let r = parseInt(point.c.r * 255) & 0xff;
            let g =  parseInt(point.c.g * 255)& 0xff;
            let b =  parseInt(point.c.b * 255)& 0xff;
            if (it < 20 )  {
                console.log(`XXXX   YYYY  p  = ${JSON.stringify(point)}   idx = ${idx}   r ${r}  g ${g}   b ${b}`);
            }
            it +=1;

            data[idx] = 127;
            data[idx+1] = 127;
            data[idx+2] = 128;
        });

         for (let i = 0; i < 120*80*4; i++) {
             imageData.data[i] = i % 255;
        }

        console.log("puttting imagedata back   ", JSON.stringify(imageData.data, null, 4));
        ctx.putImageData(imageData, 0, 0);

         // draw();
    }

    function draw( ) {
        var canvas = document.getElementById('canv');
        var ctx = canvas.getContext('2d');
        var imageData = ctx.getImageData(0,0, canvas.width, canvas.height)
        console.log(`canvas w x h ${canvas.width}  x  ${canvas.height}  `);

         for (let i = 0; i < 120*80*4; i++) {
             imageData.data[i] = i % 255;
        }

        ctx.putImageData(imageData, 0, 0);
    }




</script>
</body>
</html>
"#;
