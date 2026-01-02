<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.11" tiledversion="1.11.0" name="Blutti" tilewidth="8" tileheight="8" tilecount="256" columns="16" objectalignment="topleft">
 <image source="spritesheet.png" width="128" height="128"/>
 <tile id="0">
  <animation>
   <frame tileid="108" duration="166"/>
   <frame tileid="109" duration="166"/>
   <frame tileid="110" duration="166"/>
   <frame tileid="111" duration="166"/>
  </animation>
 </tile>
 <tile id="21">
  <properties>
   <property name="movement" type="int" value="0"/>
  </properties>
 </tile>
 <tile id="83">
  <animation>
   <frame tileid="80" duration="166"/>
   <frame tileid="81" duration="166"/>
   <frame tileid="82" duration="166"/>
   <frame tileid="83" duration="166"/>
  </animation>
 </tile>
 <tile id="116">
  <animation>
   <frame tileid="118" duration="166"/>
   <frame tileid="119" duration="166"/>
  </animation>
 </tile>
 <tile id="128" type="Monster">
  <properties>
   <property name="velocity" type="class" propertytype="Vector">
    <properties>
     <property name="x" type="float" value="1"/>
    </properties>
   </property>
  </properties>
  <animation>
   <frame tileid="128" duration="166"/>
   <frame tileid="129" duration="166"/>
  </animation>
 </tile>
 <tile id="130">
  <animation>
   <frame tileid="130" duration="166"/>
   <frame tileid="131" duration="166"/>
  </animation>
 </tile>
 <tile id="132" type="Monster">
  <properties>
   <property name="velocity" type="class" propertytype="Vector">
    <properties>
     <property name="y" type="float" value="1"/>
    </properties>
   </property>
  </properties>
  <animation>
   <frame tileid="132" duration="166"/>
   <frame tileid="133" duration="166"/>
  </animation>
 </tile>
 <tile id="134" type="Monster">
  <animation>
   <frame tileid="134" duration="166"/>
   <frame tileid="135" duration="166"/>
  </animation>
 </tile>
 <tile id="136" type="Monster">
  <animation>
   <frame tileid="136" duration="166"/>
   <frame tileid="137" duration="166"/>
  </animation>
 </tile>
 <tile id="140" type="Monster">
  <properties>
   <property name="collision" propertytype="MonsterCollider" value="None"/>
   <property name="movement" propertytype="MonsterMovement" value="Flying"/>
   <property name="reverse_sprite" type="int" value="140"/>
   <property name="sprites" type="int" value="4"/>
   <property name="velocity" type="class" propertytype="Vector">
    <properties>
     <property name="x" type="float" value="0.2"/>
    </properties>
   </property>
  </properties>
  <animation>
   <frame tileid="140" duration="333"/>
   <frame tileid="141" duration="333"/>
   <frame tileid="142" duration="333"/>
   <frame tileid="143" duration="333"/>
  </animation>
 </tile>
 <wangsets>
  <wangset name="Grass" type="mixed" tile="-1">
   <wangcolor name="Blutti tile" color="#00ff00" tile="-1" probability="1"/>
   <wangtile tileid="3" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="4" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="5" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="20" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="21" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="37" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="38" wangid="1,1,1,1,1,1,1,1"/>
   <wangtile tileid="54" wangid="1,1,1,1,1,1,1,1"/>
  </wangset>
 </wangsets>
</tileset>
