subshader "tie_cockpit_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/TieFighter/bodytex";
}

subshader "tie_cockpit_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/TieFighter/toptex";
}

subshader "tie_cockpit_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Vehicles/TieFighter/paneltex";
}

subshader "tie_cockpit_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	transparent true;
	texture "texture/Vehicles/TieFighter/fronttex";
      depthWrite false;
}

subshader "tie_cockpit_Material4" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	transparent true;
        depthWrite false;
	texture "texture/Vehicles/TieFighter/fronttex";
      depthWrite false;
}

