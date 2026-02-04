# Phoenix Framework Apps

## Homelab viewer

View created resources in Homelab and their ownership
View resources in queue to be created

## Higia

Hospital health care replacement system

### Goals

1. Data Ownership/permission by patients (Blockchain audit trail based)
2. Policy based templates for doctor reports
3. Auditable and shareable data based on trust
4. Visual selection of affected areas and symptoms
5. Faster reports for most common diagnostics

#### Goal 1

Use cases:
- Patient gives permission for Doctor to share specific data with other doctor|hospital|etc for X time
- Location based restriction of shareable data i.e if doctor logging from home then only certain data can be viewed
- Classification of fine grain data and what actions can be performed on it and who can hold it


#### Goal 2

Allow hospital to set policies for what their reports need in terms of fields;
Give freedom to doctors to have custom template as long as it complies with policies

#### Goal 3 

Allow doctors to view data from outside main hospital, but restrict it based on location, device, etc
ex: if on the personal device in a public wifi, then only least confidential data allowed (maybe patient number? schedules? consultations?)
ex: if on the hospital device in a private domicile, then you get access to more data (patient names, phone numbers, etc)
 etc

#### Goal 4

Visual models of diferent layers for the human body
Interactive for symptom visualization
Adaptable to each view based on speciality

ex:
See bone layer, click on arm, see each bones and breakages based on xray reports ordered

#### Goal 5 

Classify each data field a patient gives on different levels of confidentiality based on the impact leakage of said data can cause
For access to that data, an audit trail is publicly viewable

ex:
at <timestamp>
entity <doctor>
was given access by <entity>
to data of confidentiality <level>
for a period of <timestamp>
