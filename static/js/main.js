const navToggle = document.querySelector('#navToggle');
const nav = document.querySelector('nav');
const navIcon = document.querySelectorAll('.navIcon');
const hamburger = document.querySelector('#hamburger');
const navlinks = document.querySelectorAll('.navlinks');

navToggle.addEventListener("click", () => {
    nav.classList.toggle('open-nav');
    navIcon.forEach(icon => {
        icon.classList.toggle('hidden');
    })
})

window.addEventListener("resize", () => {
    if(document.body.clientWidth > 860) {
        nav.classList.remove('open-nav');
        navIcon.forEach(icon => {
            icon.classList.add('hidden')
        });
        hamburger.classList.remove('hidden')
    }
})

for (let i = 0; i < navlinks.length; i++) {
    navlinks[i].addEventListener("click", (evt) => {
        if(nav.classList.contains('open-nav')) {
            nav.classList.toggle('open-nav');
            navIcon.forEach(icon => {
                icon.classList.toggle('hidden');
            });
            console.log(evt.currentTarget.href);
        };
    })
}
