import { Dialog, Transition } from '@headlessui/react';
import { KeyIcon } from '@heroicons/react/24/solid';
import { Fragment, useRef, useState } from 'react';

const enterUUID = {
  title: '個人識別番号入力',
  description: '実験実施者から受け取った紙に書かれている個人識別番号を入力してください。',
  label: '個人識別番号',
  labelPlaceholder: 'ABC-1234',
  labelNoUUID: '個人識別番号を入力せずに入室する。',
  submit: '入室する',
  cancel: 'キャンセルする',
};

export interface EnterUUIDDialogProps {
  open: boolean;
  onClose: () => void;
}

export default function EnterUUID(props: EnterUUIDDialogProps) {
  const { onClose, open } = props;
  const [noUUID, setNoUUID] = useState(false);

  const handleClose = () => {
    onClose();
  };

  const cancelButtonRef = useRef(null);

  return (
    <Transition.Root show={open} as={Fragment}>
      <Dialog
        as="div"
        className="relative z-10"
        initialFocus={cancelButtonRef}
        onClose={handleClose}
      >
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-gray-500 bg-opacity-[75] transition-opacity" />
        </Transition.Child>

        <div className="fixed inset-0 z-10 overflow-y-auto">
          <div className="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
              enterTo="opacity-100 translate-y-0 sm:scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 translate-y-0 sm:scale-100"
              leaveTo="opacity-0 translate-y-4 sm:translate-y-0 sm:scale-95"
            >
              <Dialog.Panel className="relative overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg sm:p-6">
                <div>
                  <div className="mx-auto flex h-12 w-12 items-center justify-center rounded-full bg-green-100">
                    <KeyIcon className="h-6 w-6 text-gray-500" aria-hidden="true" />
                  </div>
                  <div className="mt-3 text-center sm:mt-5">
                    <Dialog.Title
                      as="h3"
                      className="text-base font-semibold leading-6 text-gray-900"
                    >
                      {enterUUID.title}
                    </Dialog.Title>
                    <div className="my-4">
                      <p className="text-left text-sm text-gray-500">{enterUUID.description}</p>
                    </div>
                  </div>
                </div>
                <form className="space-y-6" action="#">
                  <div>
                    <label
                      htmlFor="id-text"
                      className="mb-2 block text-sm font-medium text-gray-900"
                    >
                      {enterUUID.label}
                    </label>
                    <input
                      disabled={noUUID}
                      type="text"
                      name="id"
                      id="id-text"
                      className="block w-full rounded-lg border border-blue-300 bg-gray-50 p-2.5 text-base text-gray-900 focus:border-blue-500 focus:outline-blue-500 focus:ring-blue-500 disabled:bg-gray-300"
                      placeholder={enterUUID.labelPlaceholder}
                    />
                  </div>

                  <div className="flex justify-between">
                    <div className="flex items-start">
                      <div className="flex h-5 items-center">
                        <input
                          onClick={() => {
                            setNoUUID(!noUUID);
                          }}
                          id="remember"
                          type="checkbox"
                          value=""
                          className="h-4 w-4 rounded border  bg-gray-50 focus:ring-1 focus:ring-blue-500"
                        />
                      </div>
                      <label htmlFor="remember" className="ml-2 text-sm font-medium text-gray-900">
                        {enterUUID.labelNoUUID}
                      </label>
                    </div>
                  </div>
                </form>
                <div className="mt-5 sm:mt-6 sm:grid sm:grid-flow-row-dense sm:grid-cols-2 sm:gap-3">
                  <button
                    type="button"
                    className="inline-flex w-full justify-center rounded-md bg-blue-600 px-3 py-2 text-sm text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-600 sm:col-start-2"
                    onClick={handleClose}
                  >
                    {enterUUID.submit}
                  </button>
                  <button
                    type="button"
                    className="mt-3 inline-flex w-full justify-center rounded-md bg-white px-3 py-2 text-sm text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 sm:col-start-1 sm:mt-0"
                    onClick={handleClose}
                    ref={cancelButtonRef}
                  >
                    {enterUUID.cancel}
                  </button>
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition.Root>
  );
}
