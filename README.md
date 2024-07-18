# Dependency Markup Language (DML)
DMLは、アイテムやグループの定義、依存関係を管理するためのマークアップ言語です。主にゲームやアプリケーションにおけるリソース管理に適しています。

## DML言語仕様

## 基本構文

#### アイテムの定義
```dml
Item item_id:
  Translations:
    en: "英語名"
    ja: "日本語名"
```
- `item_id`: アイテムの一意な識別子。
- `en`: 英語名。
- `ja`: 日本語名。

その他の言語も自由に追加することができます。

#### グループの定義
```dml
Group group_name:
  &item_id: 数量
  &group_name
```
- `group_name`: グループの一意な識別子。
- `&item_id`: アイテムのIDと必要な数量。
- `&group_name`: 別のグループ名。グループの内容が内部的に展開されます。

#### インポート
- 全てをインポート:
  ```dml
  import ファイル名
  ```
- 特定のアイテムやグループをインポート:
  ```dml
  from ファイル名.dml import item_id, group_name
  ```

##### 使用例

#### ファイル `materials.dml`
```dml
Item Stone:
  Translations:
    en: "Stone"
    ja: "石"

Item Firestone:
  Translations:
    en: "Firestone"
    ja: "火打石"

Group BasicMaterials:
  &Stone: 1
  &Firestone: 1
```

#### ファイル `food.dml`
```dml
Item RawMeat:
  Translations:
    en: "Raw Meat"
    ja: "生肉"

Group Food:
  &RawMeat: 1
```

#### ファイル `all_materials.dml`
```dml
import materials
from food.dml import Food

Group AllMaterials:
  &BasicMaterials
  &Food
```

#### 注意点
- 各アイテムとグループは一意のIDを持つ必要があります。
- インポート文を使用して外部ファイルからアイテムやグループを読み込むことができます。
- グループ内で他のグループを参照すると、その内容が展開されます。