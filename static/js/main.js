const navToggle = document.querySelector('#navToggle');
const nav = document.querySelector('nav');
const navIcon = document.querySelectorAll('.navIcon');
const hamburger = document.querySelector('#hamburger');

navToggle.addEventListener("click", () => {
    nav.classList.toggle('open-nav');
    navIcon.forEach(icon => {
        icon.classList.toggle('hidden');
    })
})

window.addEventListener("resize", () => {
    if(document.body.clientWidth > 780) {
        nav.classList.remove('open-nav');
        navIcon.forEach(icon => {
            icon.classList.add('hidden')
        });
        hamburger.classList.remove('hidden')
    }
})
