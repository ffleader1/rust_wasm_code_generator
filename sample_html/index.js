(async () => {
    const module = await import("/wasm/rust_wasm_code_generator_lib.js");

    let input_area = document.getElementById("request")
    input_area.textContent = "curl --location 'https://dog.ceo/api/breeds/list/all'"
    let button = document.getElementById("submit_button");
    let output_area = document.getElementById("response")

    let resObj

    button.addEventListener("click", myFunction);

    async function run() {
        await module.default()
        resObj = await module.WasmCurlHandlerObj.new();
    }

    await run();

    async function myFunction() {
        let text = input_area.value
        await resObj.generate(text);
        output_area.value = resObj.get_response();
    }


})();