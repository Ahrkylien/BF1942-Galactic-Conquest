subshader "echo_map_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/buildings/SuperEcho/gmappart2";
}

subshader "echo_map_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	transparent true;
      texture "texture/buildings/SuperEcho/gmapglass";
}

