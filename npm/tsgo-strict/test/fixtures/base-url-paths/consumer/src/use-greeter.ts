import { greet } from '@lib/greeter';

// Strict violation: Type 'string' is not assignable to type 'number'
export const result: number = greet('world');
