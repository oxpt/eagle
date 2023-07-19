import PageTitle from "@/components/pageTitle";
import PageTitleInput from "@/components/pageTitleInput";
import HostDescriptionInput from "@/components/conductor/descriptionInput";
import FooterNavButton from "@/components/footerNaviButton";

import { setting } from "@/data/settings";

export default function Description() {
  return (
    <>
      <PageTitle
        title="同意文編集"
        description="以下の同意文を追加、削除、編集してください。"
      />
      <div className="rounded-lg border-2 border-dashed border-blue-100 bg-white px-1">
        <PageTitleInput
          title={setting.agreement.pageTitle.title}
          description={setting.agreement.pageTitle.discription}
        />
        <HostDescriptionInput
          researchTitle={setting.agreement.researchTitle}
          listItems={setting.agreement.listItems}
          confirm={setting.agreement.confirm}
        />
      </div>
      <FooterNavButton
        leftTitle="前へ戻る"
        leftDescription="実験順序を設定しなおす"
        leftURL="process"
        rightTitle="同意文を決定する"
        rightDescription="個別実験設定へすすむ"
        rightURL="experiment"
      />
    </>
  );
}
