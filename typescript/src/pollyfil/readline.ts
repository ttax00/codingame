import * as rl from 'readline-sync';

export function readline(): string {
	return rl.question("Input: ");
}