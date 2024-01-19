var c = document.getElementById("mcanvas");

var gx = c.getContext("2d");
gx.font = "30px Arial";
gx.textAlign = "center";
gx.textBaseline = 'middle';

var lines = [];
var last = null;
var position = [0, 0];
var points = null;
var halfplane = null;
var makesHalfplane = false;
var boundsAbove = false;

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

function clampRect(line, x) {
    let y = x * line.m + line.b;
    if (y < 0) {
        x = -line.b / line.m;
        y = 0;
    } else if (y > c.height) {
        x = (c.height - line.b) / line.m;
        y = c.height;
    }
    return [x, y];
}

function draw() {
    gx.lineWidth = 5;
    gx.clearRect(0, 0, c.width, c.height);

    if (halfplane != null) {

        gx.beginPath();

        let p1x = 0;

        let p1 = clampRect(halfplane, p1x);
        p1x = p1[0];
        let p1y = p1[1];

        gx.moveTo(p1x, p1y);

        let p2x = c.width;
        let p2 = clampRect(halfplane, p2x);
        p2x = p2[0];
        let p2y = p2[1];

        if (boundsAbove) {
            if (p1x > 0 && p1y == c.height) {
                gx.lineTo(0, c.height);
            }

            if (p1y > 0) {
                gx.lineTo(0, 0);
            }

            if (p2x == c.width || p2y > 0) {
                gx.lineTo(c.width, 0);
            }

            if (p2x < c.width && p2y == c.height) {
                gx.lineTo(c.width, c.height);
            }
        } else {
            if (p1x > 0 && p1y == 0) {
                gx.lineTo(0, 0);
            }
            if (p1y < c.height) {
                gx.lineTo(0, c.height);
            }
            if (p2y < c.height) {
                gx.lineTo(c.width, c.height);
            }
            if (p2y == 0 && p2x < c.width) {
                gx.lineTo(c.width, 0);
            }
        }

        gx.lineTo(p2x, p2y);

        gx.closePath();

        gx.fillStyle = "rgba(0, 150, 51, 0.4)";
        gx.fill();

    }

    gx.lineWidth = 5;
    gx.strokeStyle = "rgb(90, 130, 210)";
    lines.forEach((line) => {
        drawLine(line);
    });

    if (halfplane != null) {
        gx.strokeStyle = "rgb(0, 153, 51)";
        gx.lineWidth = 5;
        drawLine(halfplane);
    }

    if (last != null) {
        if (makesHalfplane) {
            gx.strokeStyle = "rgb(0, 153, 51)";
            gx.fillStyle = "rgb(0, 153, 51)";
        } else {
            gx.strokeStyle = "rgb(0, 51, 153)";
            gx.fillStyle = "rgb(0, 51, 153)";
        }

        drawLine(fromPointsToLine(last, position));
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

function makeReq() {
    fetch("http://0.0.0.0:3000/irq", {
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
            "lines": lines,
            "halfplane": halfplane,
            "bounds_above": boundsAbove
        }),
    })
        .then((response) => response.json())
        .then((data) => {
            points = data.result;
        });
}

c.addEventListener("click", function (event) {
    
    if (event.altKey) {
        boundsAbove = !boundsAbove;
        last = null;

        makeReq();
        return;
    }

    const rect = c.getBoundingClientRect();
    var p = [(event.clientX - rect.left) * 2.0, (event.clientY - rect.top) * 2.0];

    let delta = 100;

    if (last != null) {
        if (makesHalfplane != event.shiftKey) {
            last = null;
            return;
        }

        if (last[0] - delta <= p[0] && p[0] <= last[0] + delta) {
            if (last[1] - delta <= p[1] && p[1] <= last[1] + delta) {
                last = null;
                return;
            }
        }

        const new_line = fromPointsToLine(p, last);
        if (!makesHalfplane) {
            lines.push(new_line);
        } else {
            halfplane = new_line;
        }

        last = null;

        if (lines.length < 2 || halfplane == null) {
            return;
        }

        makeReq();

    } else {
        last = p;
        makesHalfplane = event.shiftKey;
        if (event.shiftKey) {
            halfplane = null;
            points = null;
        }
    }

});

c.addEventListener("mousemove", (event) => {
    const rect = c.getBoundingClientRect();
    position = [(event.clientX - rect.left) * 2.0, (event.clientY - rect.top) * 2.0];
});

c.addEventListener("dblclick", (event) => {
    if (event.altKey) {
        return;
    }
    if (event.shiftKey) {
        halfplane = null;
        points = null;
        last = null;
        return;
    }
    lines = [];
    last = null;
    points = null;
});

draw();