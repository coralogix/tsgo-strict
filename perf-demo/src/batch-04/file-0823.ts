import type { Type822 } from '../batch-04/file-0822.js';
export interface Type823 {
  id: 823;
  name: 'File823';
  next: Type822;
}

export function make823(): Type823 {
  return { id: 823, name: 'File823', next: null as unknown as Type822 };
}
