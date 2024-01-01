var c = document.getElementById("mcanvas");

var gx = c.getContext("2d");
gx.font = "30px Arial";
gx.textAlign = "center";
gx.textBaseline = 'middle';

var lines = [];
var last = null;
var position = [0, 0];
var points = null;

const pw = 5;

function fromPointsToLine(p1, p2) {
    if (p1[0] == p2[0]) {
        p1[0] += 1;
    }
    let m = (p1[1] - p2[1]) / [p1[0] - p2[0]];
    let b = p1[1] - m * p1[0];

    return { m: m, b: b };
}

function drawLine(line) {
    let p1x = 0;
    let p1y = p1x * line.m + line.b;
    let p2x = c.width;
    let p2y = p2x * line.m + line.b;
    gx.beginPath();
    gx.moveTo(p1x, p1y);
    gx.lineTo(p2x, p2y);
    gx.stroke();
}

function draw() {
    gx.clearRect(0, 0, c.width, c.height);

    gx.strokeStyle = "black";
    lines.forEach((line) => {
        drawLine(line);
    });

    if (last != null) {
        gx.strokeStyle = "rgb(0, 153, 51)";
        drawLine(fromPointsToLine(last, position));
        gx.fillStyle = "rgb(0, 153, 51)";
        gx.fillRect(last[0] - pw, last[1] - pw, pw * 2, pw * 2);
    }

    gx.fillStyle = "red";
    if (points != null) {
        points.forEach((p) => {
            gx.fillRect(p[0] - pw, p[1] - pw, pw * 2, pw * 2);
        });
    }

    setTimeout(draw, 20);
}

c.addEventListener("click", function (event) {
    const rect = c.getBoundingClientRect();
    var p = [(event.clientX - rect.left) * 2.0, (event.clientY - rect.top) * 2.0];

    let delta = 100;

    if (last != null) {

        if (last[0] - delta <= p[0] && p[0] <= last[0] + delta) {
            if (last[1] - delta <= p[1] && p[1] <= last[1] + delta) {
                last = null;
                return;
            }
        }

        lines.push(fromPointsToLine(p, last));

        last = null;

        if (lines.length < 2) {
            return;
        }

        fetch("http://0.0.0.0:3000/eip", {
            method: "POST", // *GET, POST, PUT, DELETE, etc.
            mode: "cors", // no-cors, *cors, same-origin
            // cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
            // credentials: "same-origin", // include, *same-origin, omit
            headers: {
                "Content-Type": "application/json"
            },
            redirect: "follow", // manual, *follow, error
            referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
            body: JSON.stringify({
                "lines": lines
            }),
        })
            .then((response) => response.json())
            .then((data) => {
                points = data.result;
            });
    } else {
        last = p;
    }


});

c.addEventListener("mousemove", (event) => {
    const rect = c.getBoundingClientRect();
    position = [(event.clientX - rect.left) * 2.0, (event.clientY - rect.top) * 2.0];
});

c.addEventListener("dblclick", (event) => {
    lines = [];
    last = null;
    points = null;
});

draw();