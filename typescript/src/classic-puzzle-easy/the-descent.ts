// polyfill import for codingame helpers
import { readline } from '../pollyfil/readline';

// Solution starts here

type mountain = {
    index: number,
    value: number
}
// game loop
while (true) {
    var arr: mountain[] = [];
    for (let i = 0; i < 8; i++) {
        arr[i] = {index: i, value: parseInt(readline())};
    }
    arr.sort((f,s) => f.value - s.value);
    
    console.log(arr[7].index);     // The index of the mountain to fire on.
}   


