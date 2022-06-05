import { readline } from "../pollyfil/readline";

const N: number = parseInt(readline());
let test: string[][] = []
for (let i = 0; i < N; i++) {
    const W: string = readline();
    test.push(W.split(''))
}
const LETTERS: string[] = readline().split('');
const count: Record<string, number> = {};
LETTERS.forEach(c => {
    count[c] = LETTERS.filter(ch => ch === c).length;
})

const match = []
for(let i = 0; i <test.length; i++) {
    if (test[i].every(c => LETTERS.includes(c))
        && test[i].every(c => test[i].filter(ch => ch == c).length <= count[c]))
    {
        match.push(test[i]);
    }
}

const one = ['e', 'a', 'i', 'o', 'n', 'r', 't', 'l', 's', 'u'];
const two = ['d', 'g'];
const three = ['b', 'c', 'm', 'p'];
const four = ['f', 'h', 'v', 'w', 'y'];
const five = ['k'];
const eight = ['j', 'x'];
const ten = ['q' , 'z'];

function calPoint(s: string[]): number {
    let point = 0;
    s.forEach(c => {
        if (one.includes(c)){
            point += 1;
        } else if (two.includes(c)) {
            point += 2;
        } else if (three.includes(c)) {
            point += 3;
        } else if (four.includes(c)) {
            point += 4;
        } else if (five.includes(c)){
            point +=5;
        } else if(eight.includes(c)) {
            point += 8;
        } else if (ten.includes(c)) {
            point += 10;
        }
    })
    return point
}


let hp = 0;
let current: string[] = []
match.forEach(m => {
    const point = calPoint(m);
    if (point > hp) {
        hp = point;
        current = m;
    }
})

console.log(current.join(''));