{
    try {
        var _usingCtx = _using_ctx();
        const x = _usingCtx.a(obj);
        stmt;
        const y = _usingCtx.a(obj), z = _usingCtx.a(obj);
        doSomethingWith(x, y);
    } catch (_) {
        _usingCtx.e = _;
    } finally{
        await _usingCtx.d();
    }
}
