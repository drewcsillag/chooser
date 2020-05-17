"use strict";
var __spreadArrays = (this && this.__spreadArrays) || function () {
    for (var s = 0, i = 0, il = arguments.length; i < il; i++) s += arguments[i].length;
    for (var r = Array(s), k = 0, i = 0; i < il; i++)
        for (var a = arguments[i], j = 0, jl = a.length; j < jl; j++, k++)
            r[k] = a[j];
    return r;
};
exports.__esModule = true;
var ChoiceRunner = /** @class */ (function () {
    function ChoiceRunner() {
        this.executions = [];
    }
    ChoiceRunner.prototype.run = function (fn) {
        // Run it once to populate at least one execution
        fn(new Chooser(this.executions, []));
        while (this.executions.length > 0) {
            fn(new Chooser(this.executions, this.executions.pop()));
        }
    };
    return ChoiceRunner;
}());
exports.ChoiceRunner = ChoiceRunner;
var Chooser = /** @class */ (function () {
    function Chooser(executions, preChosen) {
        this.executions = executions;
        this.preChosen = preChosen;
        this.index = 0;
        this.newChoices = [];
    }
    Chooser.prototype.choose_index = function (numArgs) {
        if (this.index < this.preChosen.length) {
            var retind = this.preChosen[this.index];
            this.index++;
            return retind;
        }
        for (var i = 1; i < numArgs; i++) {
            this.executions.push(__spreadArrays(this.preChosen, this.newChoices, [i]));
        }
        this.newChoices.push(0);
        return 0;
    };
    Chooser.prototype.choose = function (l) {
        var ind = this.choose_index(l.length);
        return l[ind];
    };
    Chooser.prototype.pick = function (l) {
        var ind = this.choose_index(l.length);
        var ret = l[ind];
        l.splice(ind, 1);
        return ret;
    };
    return Chooser;
}());
exports.Chooser = Chooser;
function test_binary_counter(c) {
    var l = [c.choose([0, 1]), c.choose([0, 1]), c.choose([0, 1])];
    console.log(l);
}
function test_solve_magic_square(c, counterbox) {
    var left = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    var square = [];
    counterbox[1]++;
    square.push(c.pick(left));
    square.push(c.pick(left));
    square.push(c.pick(left));
    if (square[0] + square[1] + square[2] !== 15) {
        return;
    }
    square.push(c.pick(left));
    square.push(c.pick(left));
    square.push(c.pick(left));
    if (square[3] + square[4] + square[5] !== 15) {
        return;
    }
    square.push(c.pick(left));
    if (square[0] + square[3] + square[6] !== 15 || square[2] + square[4] + square[6] !== 15) {
        return;
    }
    square.push(c.pick(left));
    if (square[1] + square[4] + square[7] !== 15) {
        return;
    }
    square.push(c.pick(left));
    if (square[6] + square[7] + square[8] !== 15 ||
        square[2] + square[5] + square[8] !== 15 ||
        square[0] + square[4] + square[8] !== 15) {
        return;
    }
    console.log(square.slice(0, 3));
    console.log(square.slice(3, 6));
    console.log(square.slice(6, 9));
    console.log('');
    counterbox[0] += 1;
}
var testRunner = new ChoiceRunner();
var counterBox = [0, 0];
testRunner.run(function (c) { return test_solve_magic_square(c, counterBox); });
console.log('solutions, total executions:', counterBox);
testRunner.run(test_binary_counter);
