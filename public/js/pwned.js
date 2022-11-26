document.addEventListener("DOMContentLoaded", function() {
    let formGroup = document.querySelector('#input-form-group');
    let input = document.querySelector('#password-input');
    let pwnedText = document.querySelector('#pwned-text');
    let notPwnedText = document.querySelector('#not-pwned-text');

    async function hashPassword(password) {
        let encodedPassword = new TextEncoder().encode(password);
        let hashBuffer = await crypto.subtle.digest('SHA-1', encodedPassword);
        let hashArray = Array.from(new Uint8Array(hashBuffer));
        let hashHex = hashArray.map((b) => b.toString(16).padStart(2, '0')).join('').toUpperCase();

        return hashHex;
    }

    function clearGroup() {
        if (formGroup.classList.contains('has-error')) {
            formGroup.classList.remove('has-error');
            pwnedText.classList.add('hidden');
            notPwnedText.classList.add('hidden');
        } else if (formGroup.classList.contains('has-success')) {
            formGroup.classList.remove('has-success');
            pwnedText.classList.add('hidden');
            notPwnedText.classList.add('hidden');
        }
    }

    function doRequest() {
        clearGroup();

        hashPassword(input.value)
            .then((hash) => {
                fetch('/api/' + hash)
                    .then((response) => response.text())
                    .then(function (response) {
                        if ('0' === response) {
                            formGroup.classList.add('has-success');
                            pwnedText.classList.add('hidden');
                            notPwnedText.classList.remove('hidden');
                        } else {
                            formGroup.classList.add('has-error');
                            pwnedText.classList.remove('hidden');
                            notPwnedText.classList.add('hidden');
                        }
                    }).catch(function (err) {
                    console.warn('Something went wrong.');
                });
            });
    }

    document.querySelector('#pwned-submit').addEventListener('click', () => {
        doRequest();
    });

    input.addEventListener("keyup", (event) => {
        if (0 === event.target.value.length) {
            clearGroup();
        } else if (
            13 === event.keyCode
            || "Enter" === event.key
        ) {
            doRequest();
        }
    });
});
