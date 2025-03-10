//// [assignmentCompatWithObjectMembers.ts]
// members N and M of types S and T have the same name, same accessibility, same optionality, and N is assignable M
// no errors expected
import { _ as _class_call_check } from "@swc/helpers/_/_class_call_check";
(function(SimpleTypes) {
    var S = function S() {
        "use strict";
        _class_call_check(this, S);
    };
    var T = function T() {
        "use strict";
        _class_call_check(this, T);
    };
    var s;
    var t;
    var s2;
    var t2;
    var a;
    var b;
    var a2 = {
        foo: ''
    };
    var b2 = {
        foo: ''
    };
    s = t;
    t = s;
    s = s2;
    s = a2;
    s2 = t2;
    t2 = s2;
    s2 = t;
    s2 = b;
    s2 = a2;
    a = b;
    b = a;
    a = s;
    a = s2;
    a = a2;
    a2 = b2;
    b2 = a2;
    a2 = b;
    a2 = t2;
    a2 = t;
})(SimpleTypes || (SimpleTypes = {}));
(function(ObjectTypes) {
    var S = function S() {
        "use strict";
        _class_call_check(this, S);
    };
    var T = function T() {
        "use strict";
        _class_call_check(this, T);
    };
    var s;
    var t;
    var s2;
    var t2;
    var a;
    var b;
    var a2 = {
        foo: a2
    };
    var b2 = {
        foo: b2
    };
    s = t;
    t = s;
    s = s2;
    s = a2;
    s2 = t2;
    t2 = s2;
    s2 = t;
    s2 = b;
    s2 = a2;
    a = b;
    b = a;
    a = s;
    a = s2;
    a = a2;
    a2 = b2;
    b2 = a2;
    a2 = b;
    a2 = t2;
    a2 = t;
})(ObjectTypes || (ObjectTypes = {}));
var SimpleTypes, ObjectTypes;
