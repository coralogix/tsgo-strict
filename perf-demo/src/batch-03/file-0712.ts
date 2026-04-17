import type { Type711 } from '../batch-03/file-0711.js';
export interface Type712 {
  id: 712;
  name: 'File712';
  next: Type711;
}

export function make712(): Type712 {
  return { id: 712, name: 'File712', next: null as unknown as Type711 };
}
