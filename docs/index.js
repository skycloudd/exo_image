import init, { convert } from "./exo_image.js";

init();

document.getElementById("generate-button").addEventListener("click", function runConvert() {
    var f = document.getElementById("image-upload").files[0];
    var r = new FileReader();

    r.onloadend = function (e) {
        var should_resize = document.getElementById("resize-checkbox").checked;

        // exoimage-yyyymmddhhmmss
        var lvl_name = "exoimage-" + new Date().toISOString().split('.')[0].replace(/[^\d]/gi, '');

        var result = convert(e.target.result, should_resize, lvl_name);

        var blob = new Blob([result], { type: "application/octet-stream" });

        var a = document.getElementById("download-a");
        a.href = window.URL.createObjectURL(blob);
        a.download = lvl_name + ".exolvl";

        var button = document.getElementById("download-button");
        button.disabled = false;
    }

    r.readAsDataURL(f);
}, false);
