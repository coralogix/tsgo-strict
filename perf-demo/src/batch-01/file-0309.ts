import type { Type308 } from '../batch-01/file-0308.js';
export interface Type309 {
  id: 309;
  name: 'File309';
  next: Type308;
}

export function make309(): Type309 {
  return { id: 309, name: 'File309', next: null as unknown as Type308 };
}
