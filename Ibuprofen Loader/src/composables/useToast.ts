// Toast notification composable
import { ref, type Ref } from 'vue';

export interface ToastMessage {
  id: number;
  message: string;
  type: 'success' | 'error' | 'info' | 'warning';
  duration?: number;
}

export const toasts = ref<ToastMessage[]>([]);

export let toastId = 0;

export const showToast = (message: string, type: ToastMessage['type'] = 'info', duration = 3000) => {
  const id = ++toastId;
  toasts.value.push({ id, message, type, duration });
  
  setTimeout(() => {
    toasts.value = toasts.value.filter(t => t.id !== id);
  }, duration);
};

export const showSuccessToast = (message: string, duration = 3000) => showToast(message, 'success', duration);
export const showErrorToast = (message: string, duration = 5000) => showToast(message, 'error', duration);
export const showInfoToast = (message: string, duration = 3000) => showToast(message, 'info', duration);
export const showWarningToast = (message: string, duration = 4000) => showToast(message, 'warning', duration);

export const removeToast = (id: number) => {
  toasts.value = toasts.value.filter(t => t.id !== id);
};

export function useToast() {
  return {
    showToast,
    showSuccessToast,
    showErrorToast,
    showInfoToast,
    showWarningToast,
    removeToast,
  };
}


