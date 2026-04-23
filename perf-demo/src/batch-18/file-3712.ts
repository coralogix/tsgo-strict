import type { Type3711 } from '../batch-18/file-3711.js';
export interface Type3712 {
  id: 3712;
  name: 'File3712';
  next: Type3711;
}

export function make3712(): Type3712 {
  return { id: 3712, name: 'File3712', next: null as unknown as Type3711 };
}
