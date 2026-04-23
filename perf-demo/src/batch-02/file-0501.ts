import type { Type500 } from '../batch-02/file-0500.js';
export interface Type501 {
  id: 501;
  name: 'File501';
  next: Type500;
}

export function make501(): Type501 {
  return { id: 501, name: 'File501', next: null as unknown as Type500 };
}
