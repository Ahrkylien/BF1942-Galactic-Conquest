subshader "ywing_cockpit_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.588 0.588 0.588;
	envmap false;
	texture "texture/vehicles/ywing/ywing_cockpit_id1";
}

subshader "ywing_cockpit_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	envmap true;
	texture "texture/vehicles/ywing/ywing_cockpit_id2";
}

subshader "ywing_cockpit_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	transparent true;
	depthwrite true;
	texture "texture/vehicles/ywing/ywing_glass2";
}

