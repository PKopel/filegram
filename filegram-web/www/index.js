import * as wasm from "filegram-web";


window.dragOverHandler = (ev) => {
    console.log("File(s) in drop zone");

    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();
};

window.dropHandler = (ev) => {
    console.log("File(s) dropped");

    // Prevent default behavior (Prevent file from being opened)
    ev.preventDefault();

    if (ev.dataTransfer.items) {
        // Use DataTransferItemList interface to access the file(s)
        [...ev.dataTransfer.items].forEach((item, i) => {
            // If dropped items aren't files, reject them
            if (item.kind === "file") {
                const file = item.getAsFile();
                const bytes = file.arrayBuffer();
                const encoded = wasm.encode(bytes);
                // Make a Blob from the bytes
                const blob = new Blob([encoded], { type: 'image/png' });

                // Use createObjectURL to make a URL for the blob
                const image = new Image();
                image.src = URL.createObjectURL(blob);

                document.body.appendChild(image);
            }
        });
    } else {
        // Use DataTransfer interface to access the file(s)
        [...ev.dataTransfer.files].forEach((file, i) => {
            console.log(`… file[${i}].name = ${file.name}`);
        });
    };
};
