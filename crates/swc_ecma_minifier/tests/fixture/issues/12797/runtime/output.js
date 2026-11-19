let count = 0;
class Unused {
    static set(x) {
        Unused.value = x;
    }
    static cache = ++count;
}
class Used {
    static set(x) {
        Used.value = x;
    }
    static cache = ++count;
}
Used.set("used"), console.log(count, Used.value, Used.cache);
