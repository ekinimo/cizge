export function getElemWidth(x){
    const elem = document.createElement("div");
    elem.style.width = "fit-content";
    elem.style.position = "absolute";
    elem.style.visibility = "hidden";
    elem.style.left = "-1000px";
    elem.style.right = "-1000px";

    elem.innerHTML = x;
    //let img_elem = document.getElementById("img");
    document.body.appendChild(elem);

    const foreign = document.getElementById("math");
    const width = foreign.scrollWidth;
    console.log("check width");
    console.log(width);
    elem.remove();
    return width;
}
export function getElemHeight(x){
    const elem = document.createElement("div");
    elem.style.width = "fit-content";
    elem.style.position = "absolute";
    elem.style.visibility = "hidden";
    elem.style.left = "-1000px";
    elem.style.right = "-1000px";

    elem.innerHTML = x;
    document.body.appendChild(elem);
    const width = elem.offsetHeight;
    console.log("check height");
    console.log(width);
    
    elem.remove();
    return width;
}
