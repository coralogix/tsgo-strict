import type { Type603 } from '../batch-03/file-0603.js';
export interface Type604 {
  id: 604;
  name: 'File604';
  next: Type603;
}

export function make604(): Type604 {
  return { id: 604, name: 'File604', next: null as unknown as Type603 };
}
