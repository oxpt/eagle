interface ResearchTitleProps {
  titleHead: string | null;
  title: string | null;
  discription: string | null;
}

interface ListItemsProps {
  title: string | null;
  discription: string | null;
}

export interface GuestDescriptionProps {
  researchTitle: ResearchTitleProps;
  listItems: ListItemsProps[];
}

function createMarkup(htmlFor: string) {
  let result = htmlFor.replace(/(className=)+?/g, "class=");
  return { __html: result };
}

export default function GuestDescription(props: GuestDescriptionProps) {
  const { researchTitle, listItems } = props;
  return (
    <>
      <div className="my-2 text-2xl">
        {researchTitle.titleHead}：
        <span className="font-semibold">{researchTitle.title}</span>
      </div>
      {researchTitle.discription && (
        <div
          className="my-2 text-justify  text-gray-700"
          dangerouslySetInnerHTML={createMarkup(researchTitle.discription)}
        />
      )}
      <div className="border-y-2 border-dashed border-gray-300">
        <dl className="divide-y-2 divide-dashed divide-gray-300">
          {listItems.map((elm) => {
            return (
              <div
                key={"discription_" + elm.title}
                className="sm:grid sm:grid-cols-12 sm:gap-3 sm:px-0"
              >
                <dt className="my-2 font-semibold text-gray-900 sm:col-span-3 sm:mb-8">
                  {elm.title}
                </dt>
                {elm.discription && (
                  <dd
                    className="mb-8 text-justify text-gray-700 sm:col-span-9 sm:mt-2"
                    dangerouslySetInnerHTML={createMarkup(elm.discription)}
                  />
                )}
              </div>
            );
          })}
        </dl>
      </div>
    </>
  );
}
