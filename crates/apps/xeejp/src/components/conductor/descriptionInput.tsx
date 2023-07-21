import { PlusIcon, MinusIcon } from "@heroicons/react/20/solid";

interface ResearchTitleProps {
  titleHead: string | null;
  title: string | null;
  discription: string | null;
}

interface ListItemsProps {
  title: string | null;
  discription: string | null;
}

interface ConfirmProps {
  discription: string | null;
  buttonText: string | null;
}

export interface HostDescriptionInputProps {
  researchTitle: ResearchTitleProps;
  listItems: ListItemsProps[];
  confirm: ConfirmProps;
}

function brToYn(htmlFor: string) {
  let result: string = htmlFor.replace(/<br\s*\/?>/gi, "\r\n");
  return result;
}

function countBr(htmlFor: string) {
  let result: number = (htmlFor.match(/<br\s*\/?>/gi) || []).length;
  return result + 1;
}

export default function HostDescriptionInput(props: HostDescriptionInputProps) {
  const { researchTitle, listItems, confirm } = props;
  return (
    <>
      <div className="my-2 px-1 text-2xl">
        <input
          className="my-2 rounded-lg border-0 border-blue-500 bg-gray-50 text-2xl hover:border"
          type="text"
          defaultValue={researchTitle.titleHead ? researchTitle.titleHead : ""}
          size={
            researchTitle.titleHead ? researchTitle.titleHead.length * 2 : 10
          }
        />
        ：
        <input
          className="my-2 w-full rounded-lg border-0 border-blue-500 bg-gray-50 text-2xl font-semibold hover:border"
          type="text"
          defaultValue={researchTitle.title ? researchTitle.title : ""}
          size={researchTitle.title ? researchTitle.title.length * 2 : 10}
        />
        {researchTitle.discription && (
          <textarea
            className="my-2 w-full rounded-lg border-0 border-blue-500 bg-gray-50 text-justify text-gray-700 hover:border"
            rows={5}
            defaultValue={brToYn(researchTitle.discription)}
          />
        )}
      </div>
      <div className="border-y-2 border-dashed border-gray-300">
        <div className="divide-y-2 divide-dashed divide-gray-300">
          {listItems.map((elm) => {
            return (
              <div
                key={"discription_" + elm.title}
                className="mx-1 items-start sm:grid sm:grid-cols-12 sm:gap-3 sm:px-0"
              >
                <input
                  type="text"
                  className="my-2 w-full rounded-lg border-0 border-blue-500 bg-gray-50 font-semibold text-gray-900 hover:border sm:col-span-3 sm:mb-0"
                  defaultValue={elm.title ? elm.title : ""}
                />
                {elm.discription && (
                  <textarea
                    className="w-full rounded-lg border-0 border-blue-500 bg-gray-50 text-justify text-gray-700 hover:border sm:col-span-9 sm:mt-2"
                    defaultValue={brToYn(elm.discription)}
                    rows={countBr(elm.discription)}
                  />
                )}
                <div className="col-span-12 mb-8 text-right">
                  <button
                    type="button"
                    className="mr-2 rounded-full bg-red-500 p-1 text-white hover:bg-red-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-700"
                  >
                    <MinusIcon className="h-3.5 w-3.5" aria-hidden="true" />
                  </button>
                  <button
                    type="button"
                    className="rounded-full bg-blue-500 p-1 text-white hover:bg-blue-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-700"
                  >
                    <PlusIcon className="h-3.5 w-3.5" aria-hidden="true" />
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      </div>
      <div className="mx-1 items-start sm:grid sm:grid-cols-12 sm:gap-3 sm:px-0">
        <input
          className="col-span-12 my-2 w-full rounded-lg border-0 border-blue-500 bg-gray-50 hover:border"
          type="text"
          defaultValue={confirm.discription ? confirm.discription : ""}
          size={confirm.discription ? confirm.discription.length * 2 : 10}
        />
        <input
          className="col-span-12 my-2 w-full rounded-lg border-0 border-blue-500 bg-blue-500 px-5 py-2.5 text-center text-sm font-medium text-white hover:border hover:bg-blue-700 focus:outline-none sm:col-span-4 sm:col-start-5 lg:col-span-2 lg:col-start-6"
          type="text"
          defaultValue={confirm.buttonText ? confirm.buttonText : ""}
          size={confirm.buttonText ? confirm.buttonText.length * 2 : 10}
        />
      </div>
    </>
  );
}
