var a = 1;
!(function () {
    a++;
    try {
        x();
    } catch (a) {
        var a;
    }
})();
console.log(a);
