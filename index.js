const rust = import('./markdown_parser')

rust.then(module => {
    const btn = document.getElementById('parse')
    const previewArea = document.getElementById('output')

    btn.addEventListener('click', () => {
        const input = document.getElementById('markdown').value
        previewArea.innerHTML = module.parse(input)
    })
})