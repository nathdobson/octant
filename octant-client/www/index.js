export function displayError(message){
    console.log("error = ", message);
    document.getElementById("message").appendChild(document.createTextNode(message))
    document.getElementById("notification").style.display="block"
}