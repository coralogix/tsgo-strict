import type { Type1958 } from '../batch-09/file-1958.js';
export interface Type1959 {
  id: 1959;
  name: 'File1959';
  next: Type1958;
}

export function make1959(): Type1959 {
  return { id: 1959, name: 'File1959', next: null as unknown as Type1958 };
}
