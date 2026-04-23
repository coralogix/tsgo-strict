import type { Type1711 } from '../batch-08/file-1711.js';
export interface Type1712 {
  id: 1712;
  name: 'File1712';
  next: Type1711;
}

export function make1712(): Type1712 {
  return { id: 1712, name: 'File1712', next: null as unknown as Type1711 };
}
