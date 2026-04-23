import type { Type805 } from '../batch-04/file-0805.js';
export interface Type806 {
  id: 806;
  name: 'File806';
  next: Type805;
}

export function make806(): Type806 {
  return { id: 806, name: 'File806', next: null as unknown as Type805 };
}
